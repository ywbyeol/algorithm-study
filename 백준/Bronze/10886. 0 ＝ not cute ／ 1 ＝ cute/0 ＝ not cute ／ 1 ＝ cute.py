V = int(input())
cute = sum([int(input()) for _ in range(V)])
print("Junhee is cute!" if cute > V//2 else "Junhee is not cute!")