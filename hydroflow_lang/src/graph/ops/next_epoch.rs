use super::{DelayType, OperatorConstraints, IDENTITY_WRITE_FN, RANGE_1};

#[hydroflow_internalmacro::operator_docgen]
pub const NEXT_EPOCH: OperatorConstraints = OperatorConstraints {
    name: "next_epoch",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: None,
    ports_out: None,
    num_args: 0,
    input_delaytype_fn: &|_| Some(DelayType::Epoch),
    write_fn: IDENTITY_WRITE_FN,
};