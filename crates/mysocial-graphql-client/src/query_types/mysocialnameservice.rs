// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// MySocialNameService Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveMySocialNameServiceQueryArgs"
)]
pub struct ResolveMySocialNameServiceQuery {
    #[arguments(domain: $name)]
    #[cynic(rename = "resolveMySonsAddress")]
    pub resolve_mysocialnameservice_address: Option<DomainAddress>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveMySocialNameServiceQueryArgs<'a> {
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
    variables = "DefaultMySocialNameServiceQueryArgs"
)]
pub struct DefaultMySocialNameServiceQuery {
    #[arguments(address: $address)]
    pub address: Option<AddressDefaultMySocialNameService>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DefaultMySocialNameServiceQueryArgs {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct AddressDefaultMySocialNameService {
    #[cynic(rename = "defaultMySonsName")]
    pub default_mysocialnameservice_name: Option<String>,
}
