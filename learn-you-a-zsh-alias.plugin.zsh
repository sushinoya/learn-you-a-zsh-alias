echo_alias () { 
  target/release/learn-you-zsh-alias $1
}

add-zsh-hook preexec echo_alias
