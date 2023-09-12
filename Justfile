alias r := run

run:
  cargo r --release

test:
  cargo nextest run || cargo insta review
