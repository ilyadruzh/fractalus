// #include "pgraph.h"
#include <complex.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <stdbool.h>

#define W 854
#define H 480
#define X0 427
#define Y0 240
#define PXL 0.1
#define EPS 0.1
#define PI5 0.628318530717959
#define AQUA \
    (color) { 0, 255, 255 }
#define BLACK \
    (color) { 0, 0, 0 }
#define BLUE \
    (color) { 0, 0, 255 }
#define FUCHSIA \
    (color) { 255, 0, 255 }
#define GRAY \
    (color) { 128, 128, 128 }
#define GREEN \
    (color) { 0, 128, 0 }
#define LIME \
    (color) { 0, 255, 0 }
#define MAROON \
    (color) { 128, 0, 0 }
#define NAVY \
    (color) { 0, 0, 128 }
#define OLIVE \
    (color) { 128, 128, 0 }
#define PURPLE \
    (color) { 128, 0, 128 }
#define RED \
    (color) { 255, 0, 0 }
#define SILVER \
    (color) { 192, 192, 192 }
#define TEAL \
    (color) { 0, 128, 128 }
#define WHITE \
    (color) { 255, 255, 255 }
#define YELLOW \
    (color) { 255, 255, 0 }

#define el_type point

typedef unsigned int uint;
typedef unsigned char uchar;
typedef unsigned short ushort;

typedef struct
{
    uchar red;
    uchar green;
    uchar blue;
} color;

typedef struct
{
    uint x;
    uint y;
} point;

typedef struct
{
    uint width;
    uint height;
    color cur_col;
    point cur_pnt;
    color pixels[];
} image;

struct node
{
    el_type element;
    struct node *next;
};

typedef struct node node;

node *push(node *stack, el_type element);
el_type pop(node **pstack);

image *create_image(uint width, uint height)
{
    uint wh = width * height;
    image *img = malloc(sizeof(image) + wh * sizeof(color));
    if (!img)
    {
        puts("������������ ������. ������ ��������� ����������");
        exit(1);
    }
    img->width = width;
    img->height = height;
    img->cur_col = (color){0, 0, 0};
    img->cur_pnt = (point){0, 0};
    for (uint i = 0; i < wh; i++)
        img->pixels[i] = (color){255, 255, 255};
    return img;
}

bool save_to_file(image *img, const char *filename)
{
    FILE *fp;
    if (!(fp = fopen(filename, "wb")))
        return false;
    uint w = img->width, h = img->height;
    color *pixels = img->pixels;
    char d = w % 4;
    uint z = 0;
    ushort header[27];
    for (int i = 0; i < 27; i++)
        header[i] = 0;
    header[0] = 19778;
    *(uint *)(header + 1) = (*(uint *)(header + 17) = h * (w * 3 + d)) + 54;
    header[5] = 54;
    header[7] = 40;
    *(uint *)(header + 9) = w;
    *(uint *)(header + 11) = h;
    header[13] = 1;
    header[14] = 24;
    if (fwrite(header, 2, 27, fp) != 27)
        return false;
    for (uint i = 0; i < h; i++)
    {
        for (uint j = 0; j < w; j++)
        {
            color col = pixels[i * w + j];
            if (fwrite(&(color){col.blue, col.green, col.red}, 1, 3, fp) != 3)
                return false;
        }
        if (d && fwrite(&z, 1, d, fp) != d)
            return false;
    }
    fclose(fp);
    return true;
}

color get_color(image *img, uint x, uint y)
{
    if (x < img->width && y < img->height)
        return img->pixels[y * img->width + x];
    return WHITE;
}

image *set_color(image *img, uint x, uint y, color col)
{
    if (x < img->width && y < img->height)
        img->pixels[y * img->width + x] = col;
    return img;
}

image *set_cur_col(image *img, color col)
{
    img->cur_col = col;
    return img;
}

image *set_cur_pnt(image *img, uint x, uint y)
{
    if (x < img->width && y < img->height)
        img->cur_pnt = (point){x, y};
    return img;
}

color to_color(uint colorref)
{
    uchar *p = (uchar *)&colorref;
    return (color){*(p + 2), *(p + 1), *p};
}

uint from_color(color col)
{
    uint i = 0;
    uchar *p = (uchar *)&i;
    *(p + 2) = col.red;
    *(p + 1) = col.green;
    *p = col.blue;
    return i;
}

uint smart_round(double d, bool flag)
{
    int i = d;
    if ((d - i) == 0.5)
        return flag ? i + 1 : i;
    return round(d);
}

void create_line(uint x1, uint y1, uint x2, uint y2,
                 uint width, color col, color pixels[])
{
    if (x1 == x2 && y1 == y2)
    {
        pixels[y2 * width + x2] = col;
        return;
    }
    int delta_x = x1 - x2, delta_y = y1 - y2;
    bool decreases = delta_x * delta_y < 0;
    if (abs(delta_x) > abs(delta_y))
    {
        double p = delta_y / (double)delta_x;
        double q = (int)(x1 * y2 - x2 * y1) / (double)delta_x;
        int s = delta_x > 0 ? -1 : 1;
        x2 += s;
        for (uint x = x1; x != x2; x += s)
            pixels[smart_round(p * x + q, decreases) * width + x] = col;
    }
    else
    {
        double p = delta_x / (double)delta_y;
        double q = (int)(y1 * x2 - y2 * x1) / (double)delta_y;
        int s = delta_y > 0 ? -1 : 1;
        y2 += s;
        for (uint y = y1; y != y2; y += s)
            pixels[y * width + smart_round((p * y + q), decreases)] = col;
    }
}

image *line(image *img, uint x1, uint y1, uint x2, uint y2)
{
    uint width = img->width, height = img->height;
    if (x1 >= width || y1 >= height || x2 >= width || y2 >= height)
        return img;
    create_line(x1, y1, x2, y2, width, img->cur_col, img->pixels);
    return img;
}

image *line_to(image *img, uint x, uint y)
{
    uint x1 = img->cur_pnt.x, y1 = img->cur_pnt.y;
    uint width = img->width;
    if (x >= width || y >= img->height)
        return img;
    create_line(x1, y1, x, y, width, img->cur_col, img->pixels);
    img->cur_pnt = (point){x, y};
    return img;
}

bool col_cmp(color col1, color col2)
{
    return col1.red == col2.red && col1.green == col2.green && col1.blue == col2.blue;
}

image *fill(image *img, uint x, uint y, color col)
{
    uint width = img->width, height = img->height;
    if (x >= width && y >= height)
        return img;
    color *pixels = img->pixels, old_col = pixels[y * width + x];
    if (col_cmp(col, old_col))
        return img;
    node *stack = 0;
    stack = push(stack, (point){x, y});
    do
    {
        point pnt = pop(&stack);
        uint x = pnt.x, y = pnt.y;
        if (col_cmp(pixels[y * width + x], col))
            continue;
        pixels[y * width + x] = col;
        if (x < width && col_cmp(pixels[y * width + x + 1], old_col))
            stack = push(stack, (point){x + 1, y});
        if (x > 0 && col_cmp(pixels[y * width + x - 1], old_col))
            stack = push(stack, (point){x - 1, y});
        if (y < height && col_cmp(pixels[(y + 1) * width + x], old_col))
            stack = push(stack, (point){x, y + 1});
        if (y > 0 && col_cmp(pixels[(y - 1) * width + x], old_col))
            stack = push(stack, (point){x, y - 1});
    } while (stack);
    return img;
}

image *fill_all(image *img, color col)
{
    uint wh = img->width * img->height;
    for (uint i = 0; i < wh; i++)
        img->pixels[i] = col;
    return img;
}

void void_stack()
{
    printf("������� ��������� � �������� �������� ��������������� �����\n");
    printf("������ ��������� ����������\n");
    exit(1);
}

void *pmalloc(size_t size)
{
    void *p = malloc(size);
    if (p)
        return p;
    printf("������������ ������. ������ ��������� ����������\n");
    exit(1);
}

node *push(node *stack, el_type element)
{
    node *new_node = pmalloc(sizeof(node)); //�������� ������ �� ����� ����
    new_node->element = element;            //����������� ��������
    new_node->next = stack;                 //��� �����
    return new_node;                        //���������� ����� ���������� ����
}

el_type pop(node **pstack)
{
    if (*pstack)
    {
        node *first_node = *pstack;
        *pstack = first_node->next;
        el_type element = first_node->element;
        free(first_node);
        return element;
    }
    void_stack();
}

void fractal1()
{
    image *img = create_image(W, H);
    for (int i = 0; i < W; i++)
        for (int j = 0; j < H; j++)
        {
            double x = (i - X0) * PXL;
            double y = (j - Y0) * PXL;
            complex double z = x + I * y;
            if (x || y)
            {
                double complex t;
                do
                {
                    t = z;
                    z = 0.8 * z + 0.2 * cpow(z, -4);
                }

                while (cabs(z - t) >= EPS);
                color col;
                switch ((int)(carg(z) / PI5))
                {
                case 0:
                    col = RED;
                    break;
                case 1:
                case 2:
                    col = LIME;
                    break;
                case 3:
                case 4:
                    col = BLUE;
                    break;
                case -3:
                case -4:
                    col = YELLOW;
                    break;
                case -1:
                case -2:
                    col = PURPLE;
                    break;
                }
                set_color(img, i, j, col);
            }
        }

    save_to_file(img, "result.bmp");

    free(img);
}

int main()
{
    fractal1();
    return 0;
}