use rug::{Complex, Float};
use crate::fraction::fraction;
use crate::math::{do_math, NumStr};
use crate::parse::get_func;
use crate::PrintOptions;
pub fn print_answer(input:&str, func:Vec<NumStr>, print_options:PrintOptions, prec:u32)
{
    if (input.contains('x') && !input.contains("exp"))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        return;
    }
    let num = match do_math(func, print_options.deg, prec)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("0");
            return;
        }
    };
    let sign = if num.real() != &0.0 && num.imag().is_sign_positive() { "+" } else { "" }.to_owned();
    let a = get_output(&print_options, &num, sign);
    print!("{}{}{}", a.0, a.1, if print_options.color { "\x1b[0m" } else { "" });
}
pub fn print_concurrent(unmodified_input:&str, input:&str, print_options:PrintOptions, prec:u32) -> bool
{
    if (input.contains('x') && !input.contains("exp"))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               unmodified_input);
        return false;
    }
    let func = match get_func(input, prec)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if print_options.prompt
                   {
                       if print_options.color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if print_options.color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   input);
            return false;
        }
    };
    let mut frac = false;
    let num = do_math(func, print_options.deg, prec).unwrap_or(Complex::with_val(256, 0.0));
    let a = num.real().to_f64();
    let b = num.imag().to_f64();
    let fa = fraction(num.real().clone(), print_options.tau, prec, print_options.decimal_places);
    let fb = fraction(num.imag().clone(), print_options.tau, prec, print_options.decimal_places);
    let sign = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
    let (frac_a, frac_b) = match (!fa.is_empty(), !fb.is_empty())
    {
        (true, true) =>
        {
            frac = true;
            (if a == 0.0 && b != 0.0 { "".to_string() } else { fa },
             if b == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
             })
        }
        (true, _) =>
        {
            frac = true;
            (if a == 0.0 && b != 0.0 { "".to_string() } else { fa },
             if b == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + b.to_string().as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
             })
        }
        (_, true) =>
        {
            frac = true;
            (if a == 0.0 && b != 0.0 { "".to_string() } else { a.to_string() },
             if b == 0.0
             {
                 "".to_string()
             }
             else
             {
                 sign.clone() + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
             })
        }
        _ => ("".to_string(), "".to_string()),
    };
    let output = get_output(&print_options, &num, sign);
    print!("{}{}{}{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
           if frac { "\x1b[0m\n\x1B[2K\x1B[1G" } else { "" },
           frac_a,
           frac_b,
           if !frac { "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A" } else { "" },
           output.0,
           output.1,
           if frac { "\x1b[A" } else { "" },
           if print_options.prompt
           {
               if print_options.color
               {
                   "\x1b[94m> \x1b[96m"
               }
               else
               {
                   "> "
               }
           }
           else if print_options.color
           {
               "\x1b[96m"
           }
           else
           {
               ""
           },
           unmodified_input);
    frac
}
fn get_output(print_options:&PrintOptions, num:&Complex, sign:String) -> (String, String)
{
    let mut n = String::new();
    if print_options.base != 10
    {
        (if num.real() != &0.0
         {
             n = remove_trailing_zeros(&num.real().to_string_radix(print_options.base as i32, None));
             if n.contains('e')
             {
                 n
             }
             else
             {
                 n.trim_end_matches('0').trim_end_matches('.').to_owned()
             }
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             n = remove_trailing_zeros(&num.imag().to_string_radix(print_options.base as i32, None));
             sign + &if n.contains('e') { n } else { n.trim_end_matches('0').trim_end_matches('.').to_owned() } + if print_options.color { "\x1b[93mi" } else { "i" }
         }
         else
         {
             "".to_string()
         })
    }
    else if print_options.sci
    {
        let dec = if print_options.decimal_places == 0 { 1 } else { print_options.decimal_places };
        (if num.real() != &0.0
         {
             add_commas(&remove_trailing_zeros(&format!("{:.dec$e}{}", num.real(), if print_options.color { "\x1b[0m" } else { "" })),
                        print_options.comma).replace("e0", "")
                                            .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             add_commas(&remove_trailing_zeros(&format!("{}{:.dec$e}{}", sign, num.imag(), if print_options.color { "\x1b[93mi" } else { "i" })),
                        print_options.comma).replace("e0", "")
                                            .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
         }
         else
         {
             "".to_owned()
         })
    }
    else
    {
        (if num.real() != &0.0
         {
             n = add_commas(&to_string(num.real(), print_options.decimal_places), print_options.comma);
             if n == "0" || n == "-0"
             {
                 "".to_string()
             }
             else
             {
                 n.clone()
             }
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_string()
         },
         if num.imag() != &0.0
         {
             let sign = if n == "0" { "".to_string() } else { sign };
             n = add_commas(&to_string(num.imag(), print_options.decimal_places), print_options.comma);
             if n == "0" || n == "-0"
             {
                 "".to_string()
             }
             else
             {
                 sign + &n + if print_options.color { "\x1b[93mi" } else { "i" }
             }
         }
         else
         {
             "".to_string()
         })
    }
}
fn to_string(num:&Float, decimals:usize) -> String
{
    let (neg, mut str, exp) = num.to_sign_string_exp(10, None);
    let neg = if neg { "-" } else { "" };
    if exp.is_none()
    {
        return format!("{}{}", neg, str);
    }
    let exp = exp.unwrap();
    if str.len() as i32 == exp
    {
        return format!("{}{}", neg, str);
    }
    if exp > str.len() as i32
    {
        str.push_str(&"0".repeat(exp as usize - str.len()));
    }
    let mut zeros = String::new();
    if exp < 0
    {
        zeros = "0".repeat(-exp as usize);
        str.insert_str(0, &zeros);
        str.insert(1, '.');
    }
    else
    {
        str.insert(exp as usize, '.');
    }
    let mut split = str.split('.');
    let l = split.next().unwrap();
    let mut r = split.next().unwrap().to_string();
    if r.is_empty()
    {
        return format!("{}{}", neg, l);
    }
    if r.len() > decimals
    {
        r.insert(decimals, '.');
    }
    let r = Float::with_val(num.prec(), Float::parse(&r).unwrap()).to_integer().unwrap();
    format!("{}{}.{}{}", neg, if l.is_empty() { "0" } else { l }, zeros, r).trim_end_matches(|c| c == '0')
                                                                           .trim_end_matches(|c| c == '.')
                                                                           .to_string()
}
fn add_commas(input:&str, commas:bool) -> String
{
    if !commas
    {
        return input.to_owned();
    }
    let mut split = input.split('.');
    let mut result = String::new();
    let mut count = 0;
    let mut exp = false;
    for c in split.next().unwrap().chars().rev()
    {
        if c == 'e'
        {
            exp = true;
        }
        if count == 3 && !exp
        {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    if split.clone().count() == 1
    {
        let mut result = result.chars().rev().collect::<String>();
        result.push('.');
        count = 0;
        for c in split.next().unwrap().chars()
        {
            if c == 'e'
            {
                exp = true;
            }
            if count == 3 && !exp
            {
                result.push(',');
                count = 0;
            }
            result.push(c);
            count += 1;
        }
        return result;
    }
    result.chars().rev().collect::<String>()
}
fn remove_trailing_zeros(input:&str) -> String
{
    let chars = input.chars();
    let mut result = Vec::new();
    let mut found = false;
    let mut non_zero = false;
    for c in chars.rev()
    {
        if !non_zero && found && (c == '0' || c == '.')
        {
            continue;
        }
        else
        {
            non_zero = true;
        }
        if c == 'e'
        {
            found = true;
            non_zero = false;
        }
        result.push(c);
    }
    result.iter().rev().collect::<String>()
}