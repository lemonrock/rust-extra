// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[macro_export]
macro_rules! do_while_loop
{
    (
        do
        $body:block
        while $cond:expr
    ) =>
    {
        // A while statement can use {} instead of () if the condition is a block
        while
        {
            $body;
            $cond
        }
        {
        }
    };
}
