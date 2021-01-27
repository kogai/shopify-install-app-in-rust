use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.gql",
    query_path = "gql/charge.gql",
)]

pub struct ChargeQuery;

fn charge() {
  ChargeQuery::build_query();
}
