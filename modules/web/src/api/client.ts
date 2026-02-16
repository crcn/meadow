import { GraphQLClient } from 'graphql-request'

export const client = new GraphQLClient('/graphql', {
  credentials: 'include',
})
