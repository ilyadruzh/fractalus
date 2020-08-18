#include "pgraph.h"

int main()
{
	image *img = create_image(401, 301);		//Создание изображения
	
	set_cur_col(img, (color) {153, 217, 234});	//Текущий цвет - светло-голубой
	
	for (uint i = 0; i <= 400; i += 10)			//Вертикальные
		line(img, i, 0, i, 300);				//линии
	
	for (uint i = 0; i <= 300; i += 10)			//Горизонтальные
		line(img, 0, i, 400, i);				//линии
	
	set_cur_col(img, BLACK);					//Текущий цвет - чёрный
	
	line(img, 0, 150, 400, 150);				//Ось абсцисс
	
	line_to(set_cur_pnt(img, 390, 155), 400, 150);  //Cтрелка на
	line_to(img, 390, 145);						//оси абсцисс
	
	line(img, 200, 0, 200, 300);				//Ось ординат
	
	line_to(set_cur_pnt(img, 195, 290), 200, 300);  //Cтрелка на
	line_to(img, 205, 290);						//оси ординат
	
    set_cur_col(img, RED);						//Текущий цвет - красный
    
    //Построение синусоиды
	set_cur_pnt(img, 0, (int) round (150 + 50 * sin (-4)));
    for (int x = 0; x < 401; x++)
    {
    	int y = round (150 +   50 * sin ((x-200) / 50.0));
    	line_to(img, x, y);
	}
    
    //Построение окружности
    double delta = 3.141592653589793 / 200;
    set_cur_col(img, GREEN);
    set_cur_pnt(img, 300, 150);
    for (int n = 0; n <= 400; n++)
    {
    	int x = round(200 + 100 * cos (n * delta));
    	int y = round(150 + 100 * sin (n * delta));
    	line_to(img, x, y);
	}

	fill(img, 220, 160, WHITE);				//Удаление решётки
	fill(img, 210, 155, WHITE);				//Удаление остатков решётки
	fill(img, 220, 160, SILVER);			//Заливка области серым цветом

	if (!save_to_file(img, "test.bmp"))		//Запись изображения в файл
		puts("Не удалось сохранить файл");
	
	free(img);								//Удаление изображения
	
	return 0;
}
