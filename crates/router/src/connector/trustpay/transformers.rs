
use base64::Engine;
use masking::PeekInterface;
use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};



// #[derive(Debug, Serialize, Eq, PartialEq)]
// #[serde(rename_all = "PascalCase")]
// pub struct TrustpayPaymentsRequest {
//     amount: String,
//     currency: enums::Currency,
// }

// #[derive(Debug, Eq, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct TrustpayPaymentMethod {
//     pay_method: TrustpayPaymentMethodData,
// }

// #[derive(Debug, Eq, PartialEq, Serialize)]
// #[serde(untagged)]
// pub enum TrustpayPaymentMethodData {
//     Card (TrustpayCard),
// }
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct  TrustpayCard {
    amount: String,
    currency: String,
    pan: String,
    exp: String,        
    cvv: String,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for TrustpayCard  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        match item.request.payment_method_data.clone() {
            api::PaymentMethod::Card(ccard) => {
                Ok(Self {
                    amount: item.request.amount.to_string(),
                    currency: item.request.currency.to_string().to_uppercase(),
                    pan: ccard.card_number.peek().clone(),
                    exp: format!("{}/{}", ccard.card_exp_month.peek().clone(), ccard.card_exp_year.peek().clone()),
                    cvv: ccard.card_cvc.peek().clone()
                })
            }
            _ => Err(errors::ConnectorError::NotImplemented("Payment methods".to_string()).into()),
        }
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct TrustpayAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for TrustpayAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        if let types::ConnectorAuthType::HeaderKey { api_key } = item {
            Ok(Self {
                api_key: api_key.to_string(),
            })
        } else {
            Err(errors::ConnectorError::FailedToObtainAuthType.into())
        }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[repr(i32)]
pub enum TrustpayPaymentStatus {
    Invalid = -2,
    Failed,
    Succeeded,
    #[default]
    Processing,
}

impl From<TrustpayPaymentStatus> for enums::AttemptStatus {
    fn from(item: TrustpayPaymentStatus) -> Self {
        match item {
            TrustpayPaymentStatus::Succeeded => Self::Charged,
            TrustpayPaymentStatus:: Failed => Self::Failure,
            TrustpayPaymentStatus::Invalid  => Self::Failure,
            TrustpayPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrustpayPaymentsResponse {
    status: TrustpayPaymentStatus,
    description: String,
    instanceId: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, TrustpayPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, TrustpayPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.instanceId),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct TrustpayRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for TrustpayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrustpayErrorResponse {}
