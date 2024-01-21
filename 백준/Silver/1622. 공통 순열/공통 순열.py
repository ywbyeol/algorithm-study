while True:
    try:
        a, b = input(), input()
        answer = [w * min(a.count(w), b.count(w)) for w in set(a) & set(b)]
        print("".join(sorted(answer)))
    except:
        break