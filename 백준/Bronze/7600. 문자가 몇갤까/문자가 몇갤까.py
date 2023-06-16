while 1:
    s = input()
    if s == "#":
        break
    l = [0] * 26
    for i in s.lower():
        if i.isalpha():
            l[ord(i) - 97] = 1
        else:
            continue
    print(l.count(1))