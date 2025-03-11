// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// MySocialns Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveMySocialnsQueryArgs"
)]
pub struct ResolveMySocialnsQuery {
    #[arguments(domain: $name)]
    pub resolve_mysns_address: Option<DomainAddress>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveMySocialnsQueryArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct DomainAddress {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DefaultMySocialnsNameQueryArgs"
)]
pub struct DefaultMySocialnsNameQuery {
    #[arguments(address: $address)]
    pub address: Option<AddressDefaultMySocialns>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DefaultMySocialnsNameQueryArgs {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct AddressDefaultMySocialns {
    pub default_mysns_name: Option<String>,
}
