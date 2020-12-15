def puzzle1() -> None:
    memory = {}
    mask = ''
    with open('input') as f:
        for line in f:
            lhand, rhand = line.strip().split(' = ')
            if lhand == 'mask':
                mask = rhand
            else:
                assert mask
                address = int(lhand[len('mem['):-len(']')])
                value = int(rhand)
                bin_value = f'{value:036b}'
                masked_value = ''.join(
                    vb if mb == 'X' else mb
                    for vb, mb in zip(bin_value, mask)
                )
                memory[address] = int(masked_value, 2)
    print(sum(memory.values()))


def puzzle2() -> None:
    memory = {}
    mask = ''
    with open('input') as f:
        for line in f:
            lhand, rhand = line.strip().split(' = ')
            if lhand == 'mask':
                mask = rhand
            else:
                assert mask
                address = int(lhand[len('mem['):-len(']')])
                value = int(rhand)
                bin_address = f'{address:036b}'
                masked_address = ''.join(
                    vb if mb == '0' else mb
                    for vb, mb in zip(bin_address, mask)
                )
                n_float = masked_address.count('X')
                for i in range(2**n_float):
                    true_address_bin = list(masked_address)
                    for j in reversed(range(len(true_address_bin))):
                        if true_address_bin[j] == 'X':
                            true_address_bin[j] = str(i % 2)
                            i >>= 1
                    assert not i
                    true_address = int(''.join(true_address_bin), 2)
                    memory[true_address] = value
    print(sum(memory.values()))


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
