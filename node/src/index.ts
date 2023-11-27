import express from 'express'
import { IncomingMessage } from 'http';
import logger from 'morgan'
import crypto from 'node:crypto'
import type { PostgreSqlDriver } from '@mikro-orm/postgresql'
import { MikroORM, RequestContext } from '@mikro-orm/core';
import numbers from './numbers';
import mikroOrmConfig from './mikro-orm.config';

logger.token('id', function getId(req: IncomingMessage & { id?: string }) {
  return req.id
});

const app = express();

async function init() {
  const orm = await MikroORM.init<PostgreSqlDriver>(mikroOrmConfig);

  app.use((req, res, next) => {
    RequestContext.create(orm.em, next);
  })

  app.use(function (req, _, next) {
    const id = crypto.randomUUID();
    (req as any).id = id
    console.info(`[${new Date().toISOString()}] ${req.method} ${req.originalUrl} ${id}`)
    next()
  })
  app.use(logger('[:date[iso]] :method :url :id :status :res[content-length] :response-time ms'));
  app.use(express.json());

  app.get('/', (_, res) => {
    res.send('Hello World!')
  });
  app.use(numbers);

  const port = process.env.PORT || '8080';

  app.listen(port, () => {
    console.log(`Server started at port: ${port}`)
  });
}

init()
