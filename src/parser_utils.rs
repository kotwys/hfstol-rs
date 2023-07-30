use alloc::vec::Vec;
use nom::{
    IResult,
    Parser,
    number::complete::le_u32,
    combinator::map,
};

macro_rules! parse_to_struct {
    ($in:ident, $struct:ident { $($f:ident: $e:expr,)* }) => {{
        $(let ($in, $f) = $e($in)?;)*
        Ok(($in, $struct {
            $($f),*
        }))
    }}
}

pub(crate) use parse_to_struct;

pub fn int_bool(input: &[u8]) -> IResult<&[u8], bool> {
    map(le_u32, |v| v != 0)(input)
}

pub fn parse_to_vec_n<I, O, E, F>(
    count: usize,
    mut f: F
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    F: Parser<I, O, E>
{
    move |input| {
        let mut res = Vec::with_capacity(count);
        let input = (0..count)
            .try_fold(input, |input, _| {
                let (input, index) = f.parse(input)?;
                res.push(index);
                Ok(input)
            })?;
        Ok((input, res))
    }
}
