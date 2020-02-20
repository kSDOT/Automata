
#[macro_export]
macro_rules! skip_fail {
    ($res:expr, Option<_>) => {
        match $res {
            Some(val) => val,
            None => {
                return;
            }
        }
    };
    ($res:expr, Option<_>, $return_value:expr) => {
        match $res {
            Some(val) => val,
            None => {
                return $return_value;
            }
        }
    };
    ($res:expr, Result<_>) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                return;
            }
        }
    };
    ($res:expr, Result<_>, $return_value:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                return $return_value;
            }
        }
    };
} 

#[macro_export]
macro_rules! skip_fail_loop {
    ($res:expr, Option<_>) => {
        match $res {
            Some(val) => val,
            None => {
                continue;
            }
        }
    };
    ($res:expr, Result<_>) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        }
    };
}