#!/usr/bin/python3
import sys
import re
def find(arr,word):
  for (index,a) in enumerate(arr):
    if a == word:
  #    print(f"a:{a},word:{word},i:{index}")
      return index
  return -1
if __name__=="__main__":
  name = sys.argv[1]
  with open(name, 'r') as file:
       lines = [line.strip() for line in file]
  
  NtoS = []
  EtoW = []
  
  for line in lines:
    line = line.split()
    a = line[0]
    b = line[2]
    NtoS_a = find(NtoS,a)
    EtoW_a = find(EtoW,a)
    NtoS_b = find(NtoS,b)
    EtoW_b = find(EtoW,b)
    if re.search("[NS].*",line[1]) != None:
      if NtoS_a ==-1 and NtoS_b == -1:
        NtoS.append(a)
        NtoS.append(b)
      elif NtoS_a == -1:
        NtoS.insert(NtoS_b,a)
      elif NtoS_b == -1:
        NtoS.insert(NtoS_a+1,b)
      elif NtoS_a > NtoS_b:
        print(f"CONFLIX! @ {line}")
    if re.search(".*[EW]",line[1]) != None:
      if EtoW_a ==-1 and EtoW_b == -1:
        EtoW.append(a)
        EtoW.append(b)
      elif EtoW_a == -1:
        EtoW.insert(EtoW_b,a)
      elif EtoW_b == -1:
        EtoW.insert(EtoW_a+1,b)
      elif EtoW_a > EtoW_b:
        print(f"CONFLIX! @ {line}")
  print(f'nts: {NtoS}')
  print(f'etw: {EtoW}')
