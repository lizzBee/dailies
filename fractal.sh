#!/bin/bash
: '
for i in {0..30} # rows (63)
do
  for j in {0..50} #columns (100)
  do
    echo -n "-"
  done
  echo 
done
#.1234567.12
#1.12345.123
#12.123.1234
#123.1.12345
#1234.123456
#1234.123456
#1234.12345
#1234.12345
#12345.1234
#123456.123
#1234567.12
#12345678.1
#123456789.
#1234567890.
#_________________________________________________1__________________________________________________
'
#read N

# arr=()
rChar=()
curr=()
all=()

echoChar() {
  local char="$1"
  local count="$2"
  for ((k=0; k<$count; k++)) 
    do
      echo -n $char
      done
  #printf -v char "%s" "%${count}s"
  #printf '%s\n' "${myString// /$input}"
}


function repeatChar {
  rChar=()
  local char="$1"
  local count="$2"
  for ((k=0; k<$count; k++)) 
    do
      rChar+=("$char")
    done
  #echo "rChar:${rChar[@]}"

  #printf -v char "%s" "%${count}s"
  #printf '%s\n' "${myString// /$input}"
}

function trunk {
  local n=$1
  local height=0
  #get measurements of fractal
  case $n in 
    1) height=16
  esac

  ###build 'trunk'
  for (( j=0; j<$fracs; j++ ))
    do
      repeatChar '-' $height
      arr+=("${rChar[@]}")
      arr+=('1')
      repeatChar '-' $height
      arr+=("${rChar[@]}")
    done
   half=$((height/2))
   repeatChar '-' $height-1
   #echo "rch/2:${rChar[@]}"
  right=("-" "${rChar[@]}" "1" )
  left=("1" "${rChar[@]}")
  for (( b=0; b<$height; b++ ))
    do
      for li in "${left[@]}"
         do
            echo -n "$li"
         done
      for ri in "${right[@]}"
         do
            echo -n "$ri"
         done
      echo
      l=${left[-1]}
      unset left[-1] 
      left=("$l" "${left[@]}")
      r=${right[0]}
      unset right[0] 
      right=("${right[@]}" "$r")
      #echo "b:$b< ${#right[@]}"
   done

  #echo trunk of fractal
  for (( t=0; t<$height; t++ ))
    do
      for a in "${arr[@]}"
        do
          echo -n "$a"
          done
      echo 
    done
}

function fractal {
  arr+=('1')
  arr+=repeatChar '-' $width
}

function fractal {
  local n=$1

  #if [ n -eq 0]; then return 
  #else then fractal n-1
  local fracs=`echo "2^($n-1)" | bc`
  trunk $fracs
}


fractal 1 #$N
