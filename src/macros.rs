#[macro_export]
macro_rules! open_tag {
    ($name:expr, $(($key:expr, $value:expr)),*) => {
        {
            let mut res: Vec<String> = vec![];
            res.push($name.to_string());
            $(
                res.push(format!("{}=\"{}\"", $key, $value));
            )*
            format!("<{}>", res.join(" "))
        }
    };
    ($name:expr, $attributes:expr) => {
        {
            format!("<{} {}>", $name, $attributes)
        }
    };
    ($name:expr) => {
        format!("<{}>", $name)
    };
}

#[macro_export]
macro_rules! closed_tag {
    ($name:expr, $(($key:expr, $value:expr)),*) => {
        {
            let mut res: Vec<String> = vec![];
            res.push($name.to_string());
            $(
                res.push(format!("{}=\"{}\"", $key, $value));
            )*
            format!("<{} />", res.join(" "))
        }
    };
    ($name:expr, $attributes:expr) => {
        {
            format!("<{} {} />", $name, $attributes)
        }
    };
    ($name:expr) => {
        format!("<{} />", $name)
    };
}

#[macro_export]
macro_rules! close_tag {
    ($name:expr) => {
        format!("</{}>", $name)
    };
}

#[macro_export]
macro_rules! to_style {
    ($(($key:expr, $value:expr)),*) => {
        {
            let mut res = String::from("");
            $(
                res.push(format!("{}:{};", $key, $value));
            )*
            res
        }
    };
}
