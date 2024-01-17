stack = []
stack.append(1)
stack.append(1)
while True:
	stack.pop()
	dup = stack.pop()
	stack.append(dup)
	stack.append(dup)
	dup = stack.pop()
	stack.append(dup)
	stack.append(dup)
	while True:
		dup = stack.pop()
		stack.append(dup)
		stack.append(dup)
		print(stack.pop())
		stack.append(1)
		assert len(stack) >= 2
		tmp = stack[-2]
		stack[-2] = stack[-1]
		stack[-1] = tmp
		stack.append(stack.pop()-stack.pop())
		if stack[-1] == 0:
			break
	stack.pop()
	stack.append(1)
	stack.append(stack.pop()+stack.pop())
	dup = stack.pop()
	stack.append(dup)
	stack.append(dup)
	stack.append(5)
	stack.append(stack.pop()-stack.pop())
	if stack[-1] == 0:
		break