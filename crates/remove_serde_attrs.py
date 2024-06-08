import sys


with open(sys.argv[1], 'r') as fp:
	lines = list(fp)
lines1 = []
skip = False
for lineno, line in enumerate(lines):
	line1 = line.strip()
	if line1.startswith('#[serde('):
		if not line1.endswith(')]'):
			if ')]' in line1:
				lines1.append(line)
				print(sys.argv[1], lineno)
			else:
				skip = True
	elif skip:
		if line1.endswith(')]'):
			skip = False
	else:
		lines1.append(line)
with open(sys.argv[1], 'w') as fp:
	fp.write(''.join(lines1))
