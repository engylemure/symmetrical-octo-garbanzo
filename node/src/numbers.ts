import { QueryOrder, RequestContext } from '@mikro-orm/core';
import express from 'express'
import { NumberEntity } from './entities/number';
import { z } from 'zod';
import { EntityManager } from '@mikro-orm/postgresql';

const GetNumbersQueryParams = z.object({
  isPrime: z.coerce.boolean().optional()
});

const router = express.Router();

router.get('/numbers', async function (req, res, next) {
  const query = GetNumbersQueryParams.parse(req.query);
  const em = RequestContext.getEntityManager();
  if (em) {
    res.json(await em.find(NumberEntity, query))
  } else {
    res.status(500).send()
  }
});


const PostNumbersPayload = z.object({
  value: z.coerce.number()
})

router.post('/numbers', async function (req, res, next) {
  const payload = PostNumbersPayload.safeParse(req.body);

  if (payload.success) {
    const em = RequestContext.getEntityManager() as (EntityManager | undefined);
    if (em) {
      await em.transactional(async em => {
        const numberOnDb = await em.findOne(NumberEntity, { value: payload.data.value });
        if (numberOnDb) {
          return res.json(numberOnDb)
        }
        const lastNumber = (await em.qb(NumberEntity).select('*').orderBy({ value: QueryOrder.DESC }).limit(1))?.[0]
        const numbers = sieveOfEratosthenes(payload.data.value);
        const numbersToInsert = (lastNumber ? numbers.filter((_, value) => value > lastNumber.value) : numbers).map((isPrime, value) => ({
          isPrime,
          value
        }))
        const allInsertedNumbers = await em.insertMany<NumberEntity>(NumberEntity, numbersToInsert);
        if (!allInsertedNumbers) {
          return res.json((await em.qb(NumberEntity).select('*').orderBy({ value: QueryOrder.DESC }).limit(1))?.[0])
        }
        const lastInsertedId = allInsertedNumbers[allInsertedNumbers.length - 1];
        res.json(lastInsertedId ? await em.findOne(NumberEntity, { id: lastInsertedId }) : lastNumber)
      })
    } else {
      res.status(500).send()
    }
  } else {
    res.status(422).send(payload)
  }
});

function sieveOfEratosthenes(num: number) {
  const primes: boolean[] = (new Array(num + 1)).fill(true)
  let p = 2;
  while (p * p <= num + 1) {
    if (primes[p]) {
      for (let i = p * p; i < num + 1; i += p) {
        primes[i] = false
      }
    }
    p += 1;
  }
  return primes;
}

export default router;