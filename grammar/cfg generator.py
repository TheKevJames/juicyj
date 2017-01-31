import re

def generate_cfg(grammarfile, outfile):
	rules = open(grammarfile, 'r')
	w = open(outfile, 'w')

	termlist = set()
	nontermlist = set()
	ruleset = set()

	pat = re.compile('(\w+):')
	cur_non_term = ""

	for line in rules:
		if line == "\n":
			continue
		try:
			cur_non_term = pat.match(line).groups()[0]
			nontermlist.add(cur_non_term)
		except:
			if line.strip() == "\\epsilon":
				ruleset.add(cur_non_term)
			else:
				ruleset.add(cur_non_term + " " + line.strip())
				for x in line.strip().split(' '):
					termlist.add(x)

	termlist = termlist - nontermlist

	w.write(str(len(termlist)) + '\n')
	w.write('\n'.join(termlist))
	w.write('\n')
	
	w.write(str(len(nontermlist)) + '\n')
	w.write('\n'.join(nontermlist))
	w.write('\n')

	w.write('Start\n')

	w.write(str(len(ruleset)) + '\n')
	w.write('\n'.join(ruleset))

	w.close()


generate_cfg("grammar/joos.grammar", "grammar/joos.cfg")
