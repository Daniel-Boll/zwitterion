alias r := run

run file:
  cargo r --release -- {{file}}

test:
  cargo nextest run || cargo insta review
