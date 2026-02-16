import { GraphQLClient } from 'graphql-request'

export const client = new GraphQLClient(`${window.location.origin}/graphql`, {
  credentials: 'include',
})
