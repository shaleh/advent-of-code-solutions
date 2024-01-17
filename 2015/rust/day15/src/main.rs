use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace0};
use nom::combinator::{map_res, opt, recognize};
use nom::error::ParseError;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn alpha(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse()
    })(input)
}

fn colon(input: &str) -> IResult<&str, char> {
    char(':')(input)
}

fn comma(input: &str) -> IResult<&str, char> {
    char(',')(input)
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn parse(input: &str) -> Self {
        let (input, _) = alpha(input).unwrap();
        let (input, _) = colon(input).unwrap();
        let (input, (_, capacity)) = pair(ws(tag("capacity")), ws(number))(input).unwrap();
        let (input, _) = comma(input).unwrap();
        let (input, (_, durability)) = pair(ws(tag("durability")), ws(number))(input).unwrap();
        let (input, _) = comma(input).unwrap();
        let (input, (_, flavor)) = pair(ws(tag("flavor")), ws(number))(input).unwrap();
        let (input, _) = comma(input).unwrap();
        let (input, (_, texture)) = pair(ws(tag("texture")), ws(number))(input).unwrap();
        let (input, _) = comma(input).unwrap();
        let (_, (_, calories)) = pair(ws(tag("calories")), ws(number))(input).unwrap();

        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }

    fn is_valid(&self) -> bool {
        self.capacity > 0
            && self.durability > 0
            && self.flavor > 0
            && self.texture > 0
            && self.calories > 0
    }
}

fn score(ingredients: &[Ingredient], spoons: &[i64]) -> Option<(i64, i64)> {
    let computed = ingredients
        .iter()
        .zip(spoons)
        .map(|(ingredient, number)| {
            Ingredient {
                capacity: ingredient.capacity * number,
                durability: ingredient.durability * number,
                flavor: ingredient.flavor * number,
                texture: ingredient.texture * number,
                calories: ingredient.calories * number,
            }
        })
        .fold(
            Ingredient {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            },
            |acc, ingredient| Ingredient {
                capacity: acc.capacity + ingredient.capacity,
                durability: acc.durability + ingredient.durability,
                flavor: acc.flavor + ingredient.flavor,
                texture: acc.texture + ingredient.texture,
                calories: acc.calories + ingredient.calories,
            },
        );

    if computed.is_valid() {
        Some((
            computed.capacity * computed.durability * computed.texture * computed.flavor,
            computed.calories,
        ))
    } else {
        None
    }
}

fn main() {
    let data = include_str!("input");
    let ingredients: Vec<Ingredient> = data.lines().map(Ingredient::parse).collect();
    let permutations: Vec<_> = (0..ingredients.len())
        .map(|_| 0..100)
        .multi_cartesian_product()
        .filter(|items| items.iter().sum::<i64>() == 100)
        .collect();
    println!("Permutations to check {}", permutations.len());
    let max_ignoring_calories = permutations
        .iter()
        .filter_map(|spoons| score(&ingredients, spoons).map(|(value, _)| value))
        .max();
    println!("{:?}", max_ignoring_calories);
    let max_with_500_calories = permutations
        .iter()
        .filter_map(|spoons| {
            score(&ingredients, spoons)
                .and_then(|(value, calories)| if calories == 500 { Some(value) } else { None })
        })
        .max();
    println!("{:?}", max_with_500_calories);
}
