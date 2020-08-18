#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <stdbool.h>

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

#define AQUA (color) {0, 255, 255}
#define BLACK (color) {0, 0, 0}
#define BLUE (color) {0, 0, 255}
#define FUCHSIA (color) {255, 0, 255}
#define GRAY (color) {128, 128, 128}
#define GREEN (color) {0, 128, 0}
#define LIME (color) {0, 255, 0}
#define MAROON (color) {128, 0, 0}
#define NAVY (color) {0, 0, 128}
#define OLIVE (color) {128, 128, 0}
#define PURPLE (color) {128, 0, 128}
#define RED (color) {255, 0, 0}
#define SILVER (color) {192, 192, 192}
#define TEAL (color) {0, 128, 128}
#define WHITE (color) {255, 255, 255}
#define YELLOW (color) {255, 255, 0}

image *create_image(uint width, uint height);
bool save_to_file(image *img, const char *filename);
image *set_color(image *img, uint x, uint y, color col);
color get_color(image *img, uint x, uint y);
image *set_cur_col(image *img, color col);
image *set_cur_pnt(image *img, uint x, uint y);
color to_color(uint colorref);
uint from_color(color col);
image *line(image *img, uint x1, uint y1, uint x2, uint y2);
image *line_to(image *img, uint x, uint y);
image *fill(image *img, uint x, uint y, color col);
image *fill_all(image *img, color col);
