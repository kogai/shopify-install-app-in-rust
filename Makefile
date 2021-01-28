ngrok:
	npx ngrok start dev \
		--authtoken "$(NGROK_AUTHTOKEN)" \
		--config $(CURDIR)/ngrok.yml

.PHONY: gql/schema.json
gql/schema.json:
	npx apollo service:download \
		--header="X-Shopify-Access-Token: $$SHOPIFY_ADMIN_API_GRAPHQL_SCHEMA_SECRET" \
		--endpoint="https://graphql-schema-granted.myshopify.com/admin/api/2020-07/graphql.json" \
		./gql/schema.json

db:
	docker-compose run --rm database psql -h database -U postgres -d postgres
