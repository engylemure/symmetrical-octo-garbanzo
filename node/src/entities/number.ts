import { Entity, PrimaryKey, Property } from "@mikro-orm/core";

@Entity({
  tableName: 'numbers'
})
export class NumberEntity {
  @PrimaryKey()
  id!: number
  @Property()
  value!: number
  @Property({
    fieldName: 'is_prime'
  })
  isPrime!: boolean
}
