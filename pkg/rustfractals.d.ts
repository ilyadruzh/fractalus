/* tslint:disable */
export function mul(arg0: Complex, arg1: Complex): Complex;

export function add(arg0: Complex, arg1: Complex): Complex;

export function sub(arg0: Complex, arg1: Complex): Complex;

export function sub_f64(arg0: Complex, arg1: number): Complex;

export function abs(arg0: Complex): number;

export function arg(arg0: Complex): number;

export function scale(arg0: Complex, arg1: number): Complex;

export function norm_sqr(arg0: Complex): number;

export function div(arg0: Complex, arg1: Complex): Complex;

export function zfunc(arg0: Complex): Complex;

export function dfunc(arg0: Complex): Complex;

export function wasmdraw(arg0: number, arg1: number, arg2: number): Uint32Array;

export function i_to_u(arg0: number, arg1: number): number;

export class Complex {
free(): void;
re: number
im: number

}
