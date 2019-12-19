#!/usr/bin/env python3

def split_image_into_layers(data, width, height):
    layers = []
    for i in range(len(data) // (width * height)):
        layer = []
        for row in range(height):
            layer.append(data[((i * (width * height)) + (row * width)):(i * (width * height)) + ((row + 1) * width)])

        layers.append(layer)
    return layers


def render_image(layers):
    layer_count = len(layers)
    image = []

    # import pdb; pdb.set_trace()

    for j in range(len(layers[0])):
        row = []
        for i in range(len(layers[0][0])):
            for l in range(layer_count):
                value = layers[l][j][i]
                if value in ['0', '1']:
                    row.append(value)
                    break
        image.append(row)

    return image


def print_image(layers):
    for row in layers:
        print(''.join(row))


def image_string(layers):
    return ''.join(''.join(row) for row in layers)


def count_on_layer(layer, number):
    return sum(row.count(number) for row in layer)


def main(inputfile):
    with open(inputfile) as fp:
        data = fp.read()
    layers = split_image_into_layers(data, 25, 6)
    layers_by_zero_count = sorted([(i, count_on_layer(layers[i], '0')) for i in range(len(layers))], key=lambda x: x[1])
    fewest_zeroes = layers[layers_by_zero_count[0][0]]
    print(count_on_layer(fewest_zeroes, '1') * count_on_layer(fewest_zeroes, '2'))
    rendered = render_image(layers)
    print_image(rendered)
    print()
    print(image_string(rendered))
    print()
    layers = split_image_into_layers('0222112222120000', 2, 2)
    rendered = render_image(layers)
    print_image(rendered)
    print()
    print(image_string(rendered))


if __name__ == '__main__':
    import sys
    main(sys.argv[1])
