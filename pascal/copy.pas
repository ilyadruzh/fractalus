
Program N3;

Uses Graph, Crt;

Type 
  Complex = Record
    x : Real;
    y : Real;
  End;

Const 
  iter = 50;
  max  = 1e+6;
  min  = 1e-6;

Var 
  z, t, d : Complex;
  p       :  Real;
  x, y, n : Integer;
  Cancel  : Boolean;
  gd, gm  : Integer;
  mx, my  : Integer;

Begin
  Cancel := False;
  Randomize;
  gd := Detect;
  InitGraph(gd,gm,'c:\bp\bgi');
  Mx := GetMaxX Div 2;
  My := GetMaxY Div 2;
  For y := -my To my Do
    For x := -mx To mx Do
      Begin
        n := 0;
        z.x := X * 0.005;
        z.y := Y * 0.005;
        d := z;
        While (sqr(z.x)+sqr(z.y) < max) And (sqr(d.x)+sqr(d.y) > min)
              And (n < iter) Do
          Begin
            t := z;
                {z^3 - 1}
            p := sqr(sqr(t.x)+sqr(t.y));
            z.x := 2/3*t.x + (sqr(t.x)-sqr(t.y))/(3*p);
            z.y := 2/3*t.y*(1-t.x/p);{}
            d.x := abs(t.x - z.x);
            d.y := abs(t.y - z.y);
            Inc(n);
            If keypressed Then
              Cancel := true;
          End;
        PutPixel(mx + x,my + y,16 - (n Mod 16));
        If cancel Then exit;
      End;
  Readkey;
  CloseGraph;
End.
