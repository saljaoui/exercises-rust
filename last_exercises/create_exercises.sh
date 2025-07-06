#!/bin/bash

# List of all exercise names
exercises=(
  min_and_max count_factorial_steps matrix_multiplication
  reverse_it counting_words smallest modify_letter
  partial_sums inv_pyramid previousprime nextprime profanity_filter prime_checker scytale_decoder
  insertion_sort rpn rot21 order_books matrix_determinant
  office_worker blood_types_s
  matrix_display queens lunch_queue
  drop_the_blog filter_table display_table
  flat_tree
  brackets_matching brain_fuck
)

# Create each exercise as a Rust binary crate
for ex in "${exercises[@]}"; do
  echo "Creating: $ex"
  cargo new "$ex" --lib
  mkdir -p "$ex/src/bin"
  echo "fn main() {\n    println!(\"Running $ex\");\n}" > "$ex/src/bin/main.rs"
done
