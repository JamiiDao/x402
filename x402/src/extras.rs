use crate::BorrowedStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PaymentRequestExtras<'x> {
    name: &'x dyn BorrowedStr,
    version: &'x dyn BorrowedStr,
    fee_payer: &'x dyn BorrowedStr,
}

impl<'x> PaymentRequestExtras<'x> {
    pub fn to_json(&self) -> jzon::JsonValue {
        jzon::object! {
            name: self.name.as_ref(),
            version: self.version.as_ref(),
            feePayer: self.fee_payer.as_ref()
        }
    }
}
