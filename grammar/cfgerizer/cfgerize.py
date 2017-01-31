#!/usr/bin/env python3
import re
import sys


NON_TERM = re.compile(r'(\w+):')


def main(infile, outfile):
    termlist = set()
    nontermlist = set()
    ruleset = set()

    cur_non_term = None
    with open(infile, 'r') as rules:
        for line in rules:
            if line == '\n':
                continue

            try:
                cur_non_term = NON_TERM.match(line).groups()[0]
                nontermlist.add(cur_non_term)
            except AttributeError:
                line = line.strip()
                if line == '\\epsilon':
                    ruleset.add(cur_non_term)
                    continue

                ruleset.add(cur_non_term + ' ' + line)
                for x in line.split():
                    termlist.add(x)

    termlist = termlist - nontermlist

    with open(outfile, 'w') as w:
        w.write(str(len(termlist)))
        w.write('\n')
        w.write('\n'.join(sorted(termlist)))
        w.write('\n')

        w.write(str(len(nontermlist)))
        w.write('\n')
        w.write('\n'.join(sorted(nontermlist)))
        w.write('\n')

        w.write('Start\n')

        w.write(str(len(ruleset)))
        w.write('\n')
        w.write('\n'.join(sorted(ruleset)))


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print('usage: {} <infile> <outfile>'.format(sys.argv[0]))
        sys.exit(1)

    main(sys.argv[1], sys.argv[2])
