
Program Newton;

// Uses GraphABC,Utils;

Type Complex = Record
  x : Real;
  y : Real;
End;

Const iter = 50;
  max  = 1e+6;
  min  = 1e-6;

Var  z, t, d : Complex;
  y, x, p : Real;
  xc, yc, n : Integer;
  mx, my  : real;
  x0,xn,y0,yn : real;

{Функция для перевода кода цвета в константу}
Function IntToColor(Const kode: integer): color;
Begin
  Case kode Of 
    0: result := clBlack;
    1: result := clBlue;
    2: result := clGreen;
    3: result := clCyan;
    4: result := clRed;
    5: result := clDarkMagenta;
    6: result := clBrown;
    7: result := clLightGray;
    8: result := clDarkGray;
    9: result := clLightBlue;
    10: result := clLightGreen;
    11: result := clLightCyan;
    12: result := clLightPink;
    13: result := clMagenta;
    14: result := clYellow;
    15: result := clWhite;
  End;
  IntToColor := result;
End;
Begin
  SetWindowSize(790,500);
  Window.Title := 'Фрактал Ньютона';
  Randomize;
  
{дипазон по х,у из вашего варианта}
  x0 := -0.9;
  xn := 0.9;
  y0 := -0.8;
  yn := 0.8;

{начало экранных координат, смещено влево и вверх}
  xc := round(790*(-x0)/(xn-x0));
  yc := round(500*(-y0)/(yn-y0));
{масштаб для перевода координат в экранные}
  mx := xc/(-x0);
  my := yc/-y0;
  y := y0;{цикл по у}
  Repeat
    x := x0; {цикл по х}
    Repeat
      n := 0;
      z.x := x;
      z.y := y;
      d := z;
      While (sqr(z.x)+sqr(z.y) < max) And (sqr(d.x)+sqr(d.y) > min)
            And (n < iter) Do
        Begin
          t := z;
          p := sqr(sqr(t.x)+sqr(t.y));
          z.x := 2/3*t.x + (sqr(t.x)-sqr(t.y))/(3*p);
          z.y := 2/3*t.y*(1-t.x/p);
          d.x := abs(t.x - z.x);
          d.y := abs(t.y - z.y);
          Inc(n);
        End;
      PutPixel(xc + round(x*mx),yc + round(y*my),IntToColor(16 - (n Mod 16)));
      x := x+0.001;
{шаг по x: чем больше шаг, тем быстрее рисуется, но менее качественная картинка}
    Until x>xn;
    y := y+0.001; {шаг по y}
  Until y>yn;
End.
