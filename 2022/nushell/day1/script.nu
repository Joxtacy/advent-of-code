open input.txt |
split row "\n\n" |
each { |it| $it | split row "\n" } |
each { |it| $it | into int } |
each { |it| $it | math sum } |
math max

open input.txt |
split row "\n\n" |
each { |it| $it | split row "\n" } |
each { |it| $it | into int } |
each { |it| $it | math sum } |
sort |
reverse |
range 0..2 |
math sum
