# serverless-cache-rust

An implementation of a cache backend for a GraphQL apollo server based upon the [KeyValueCache interface](https://www.apollographql.com/docs/apollo-server/performance/cache-backends/) in Apollo Server using [Rust](https://www.rust-lang.org/), [Cloudflare Workers](https://workers.cloudflare.com/) and [Cloudflare KV](https://developers.cloudflare.com/workers/learning/how-kv-works/).

## Usage

```bash
# Install the wrangler CLI
yarn

# start the development server
yarn dev

# dry run the deployment
yarn build

# deploy the project
yarn deploy
```
