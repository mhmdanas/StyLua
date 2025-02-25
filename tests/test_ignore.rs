use stylua_lib::{format_code, Config, OutputVerification};

fn format(input: &str) -> String {
    format_code(input, Config::default(), None, OutputVerification::None).unwrap()
}

#[test]
fn test_singleline_ignore() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore
local bar   =     baz
            "###
        ),
        @r###"
    local foo = bar
    -- stylua: ignore
    local bar   =     baz
    "###
    );
}

#[test]
fn test_singleline_ignore_2() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore
local bar   =     baz
local bar   =     baz
            "###
        ),
        @r###"
    local foo = bar
    -- stylua: ignore
    local bar   =     baz
    local bar = baz
    "###
    );
}

#[test]
fn test_singleline_ignore_last_stmt() {
    insta::assert_snapshot!(
        format(
            r###"-- stylua: ignore
return      "hi"
            "###
        ),
        @r###"
    -- stylua: ignore
    return      "hi"
    "###
    );
}

#[test]
fn test_singleline_ignore_stmt_block() {
    insta::assert_snapshot!(
        r###"local   x     = 1
-- stylua: ignore
function foo   ()
    return    x +    1
end"###, @r###"
    local   x     = 1
    -- stylua: ignore
    function foo   ()
        return    x +    1
    end
    "###
    )
}

#[test]
fn test_multiline_block_ignore() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    -- stylua: ignore start
    local bar   =     baz
    -- stylua: ignore end
    local bar = baz
    "###);
}

#[test]
fn test_multiline_block_ignore_2() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    -- stylua: ignore start
    local bar   =     baz
    local bar   =     baz
    -- stylua: ignore end
    local bar = baz
    "###);
}

#[test]
fn test_multiline_block_ignore_no_ending() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
local bar   =     baz
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    -- stylua: ignore start
    local bar   =     baz
    local bar   =     baz
    local bar   =     baz
    "###);
}

#[test]
fn test_multiline_block_ignore_no_starting() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
local bar   =     baz
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    local bar = baz
    local bar = baz
    -- stylua: ignore end
    local bar = baz
    "###);
}

#[test]
fn test_multiline_block_ignore_block_scope() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
do
    -- stylua: ignore start
    local bar   =     baz
    -- stylua: ignore end
    local bar   =     baz
end
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    do
        -- stylua: ignore start
        local bar   =     baz
    	-- stylua: ignore end
    	local bar = baz
    end
    local bar = baz
    "###);
}

#[test]
fn test_multiline_block_ignore_block_scope_no_ending() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
do
    -- stylua: ignore start
    local bar   =     baz
    local bar   =     baz
end
local bar   =     baz
"###
        ),
    @r###"
    local foo = bar
    do
        -- stylua: ignore start
        local bar   =     baz
        local bar   =     baz
    end
    local bar = baz
    "###);
}

#[test]
fn test_multiline_block_ignore_multiple_comments_in_leading_trivia() {
    insta::assert_snapshot!(
        format(
            r###"--stylua: ignore start
local a   =   1
--stylua: ignore end

--stylua: ignore start
local b   =   2
--stylua: ignore end

--stylua: ignore start
local c   =   3
--stylua: ignore end

-- Some very large comment

--stylua: ignore start
local d   =   4
--stylua: ignore end
"###
        ),
    @r###"
    --stylua: ignore start
    local a   =   1
    --stylua: ignore end

    --stylua: ignore start
    local b   =   2
    --stylua: ignore end

    --stylua: ignore start
    local c   =   3
    --stylua: ignore end

    -- Some very large comment

    --stylua: ignore start
    local d   =   4
    --stylua: ignore end
    "###
    )
}
