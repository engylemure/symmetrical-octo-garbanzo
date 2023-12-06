import { Options } from "@mikro-orm/postgresql"
import { TsMorphMetadataProvider } from "@mikro-orm/reflection";

const options: Options = {
  metadataProvider: TsMorphMetadataProvider,
  entities: ['./build/entities/**/*.js'],
  entitiesTs: ['./src/entities/**/*.ts'], // path to our TS entities (src), relative to `baseDir`
  type: 'postgresql',
  dbName: process.env.DB_NAME,
  host: process.env.DB_HOST,
  user: process.env.DB_USER,
  password: process.env.DB_PASSWORD,
  // debug: process.env.NODE_ENV === 'dev'
}

export default options;