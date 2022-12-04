if __name__ == '__main__':

    with open('../inputs/problem4/input.txt') as f:
        lines = f.read().strip().split('\n')

        total = 0
        for line in lines:
            first, last = line.split(',')
            f_start, f_end, l_start, l_end = (
                list(map(int, first.split('-'))) +
                list(map(int, last.split('-')))
            )
            print(line, end=' : ')

            if (
                (f_start <= l_start and l_end <= f_end) or
                (l_start <= f_start and f_end <= l_end)
            ):
                total += 1
                print('Contains')
            else:
                print('Does Not Contain')
