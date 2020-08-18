#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Sep 28 05:53:07 2019

Newton-Fractal example code. 
Solves the Newton-method for a given function
on a grid of complex initial guesses.
Produces a color-coded graph of the fractal.
Includes several presets for function, but is easily extendable.

@author: Andreas Krassnigg, ComputingSkillSet.com/about/

This work is licensed under a 
Creative Commons Attribution-ShareAlike 4.0 International License.
More information: https://creativecommons.org/licenses/by-sa/4.0/

"""

import matplotlib.pyplot as plt
import numpy as np
import datetime


# initialize a dictionary of list of the roots
rootlist = {}

# function definitions to evaluate f/f' at x. Add your own as needed.
# each function definition must include a list of the roots of this function
# root list can be in any order and restricted to finite number

# example 1: polynomial function with four roots


def npe1(x):
    return (x**2-1)*(x**2+1)/(2*x*(x**2-1)+2*x*(x**2+1))


rootlist['npe1'] = [-1, 1, -1j, 1j]


# example 2: function with three roots on the unit circle
def npe2(x):
    return (x**3-1)/(3*x**2)


rootlist['npe2'] = [-.5-0.8660254037844386j, -.5+0.8660254037844386j, 1]


# example 2: function with twelve roots on the unit circle
def npe3(x):
    return (x**12-1)/(12*x**11)


rootlist['npe3'] = [-.5-0.8660254037844386j, -.5+0.8660254037844386j, .5-0.8660254037844386j, .5+0.8660254037844386j, -.5j -
                    0.8660254037844386, -.5j+0.8660254037844386, .5j-0.8660254037844386, .5j+0.8660254037844386, 1, -1, 1.j, -1.j]


# example 7: function with four roots, all real
def npe7(x):
    return (x+2.)*(x+1.5)*(x-0.5)*(x-2.)/((x+1.5)*(x-0.5)*(x-2.) + (x+2.)*(x-0.5)*(x-2.) + (x+2.)*(x+1.5)*(x-2.) + (x+2.)*(x+1.5)*(x-0.5))


rootlist['npe7'] = [-2, -1.5, 0.5, 2]


# example 9: function with four roots, one multiple
def npe9(x):
    return (x+2)*(x+1.5)**2*(x-0.5)*(x-2)/((x+1.5)**2*(x-0.5)*(x-2) + 2*(x+2)*(x+1.5)*(x-0.5)*(x-2) + (x+2)*(x+1.5)**2*(x-2) + (x+2)*(x+1.5)**2*(x-0.5))


rootlist['npe9'] = [-2, -1.5, 0.5, 2]


# example 10: sine function
def npe10(x):
    return np.tan(x)


rootlist['npe10'] = [0]
for i in range(1, 10):
    rootlist['npe10'].extend([i*np.pi, -i*np.pi])


# define a function that can id a root from the rootlist
def id_root(zl, rlist):
    findgoal = 1.e-10 * np.ones(len(zl))
    rootid = -1 * np.ones(len(zl))
    for r in rlist:
        # check for closeness to each root in the list
        rootid = np.where(np.abs(zl-r * np.ones(len(zl))) <
                          findgoal, np.ones(len(zl)) * rlist.index(r), rootid)

    return rootid


# define parameters for plotting - adjust these as needed for your function
# define left and right boundaries for plotting
# for overview plots:
interval_left = -2.1
interval_right = 2.1
interval_down = -2.1
interval_up = 2.1

# for detailed plots (adjust as needed):
#interval_left = 1.15
#interval_right = 2.
#interval_down = -0.25
#interval_up = 0.25

# set number of grid points on x and y axes for plotting
# use 100 for testing plotting ranges, 1000 for nice plots and 2000 for nicer plots
num_x = 1000
num_y = 1000

# set desired precision and max number of iterations
# keep precision goal smaller than findgoal (root matching) above
prec_goal = 1.e-11
# max number of iterations. Is being used in a vectorized way.
# 50 is a good minimal value, sometimes you need 500 or more
nmax = 200

# timing
print('Started computation at '+str(datetime.datetime.now()))


# define x and y grids of points for computation and plotting the fractal
xvals = np.linspace(interval_left, interval_right, num=num_x)
yvals = np.linspace(interval_down, interval_up, num=num_y)


# the following defines function to solve and plot. Jump to end of code to choose function
def plot_newton_fractal(func_string, perfom_shading=False):

    # create complex list of points from x and y values
    zlist = []
    for x in xvals:
        for y in yvals:
            zlist.append(x + 1j*y)

    # initialize the arrays for results, differences, loop counters
    reslist = np.array(zlist)
    reldiff = np.ones(len(reslist))
    counter = np.zeros(len(reslist)).astype(int)
    # initialize overall counter for controlling the while loop
    overallcounter = 0
    # vectorize the precision goal
    prec_goal_list = np.ones(len(reslist)) * prec_goal
    # iterate while precision goal is not met - vectorized
    while np.any(reldiff) > prec_goal and overallcounter < nmax:

        # call function as defined above and
        # compute iteration step, new x_i, and relative difference
        diff = eval(func_string+'(reslist)')
        z1list = reslist - diff
        reldiff = np.abs(diff/reslist)

        # reset the iteration
        reslist = z1list

        # increase the vectorized counter at each point, or not (if converged)
        counter = counter + np.greater(reldiff, prec_goal_list)
        # increase the control counter
        overallcounter += 1

    # get the converged roots matched up with those predefined in the root list
    nroot = id_root(z1list, rootlist[func_string]).astype(int)

    # add information about number of iterations to the rood id for shaded plotting
    if perfom_shading == True:
        nroot = nroot - 0.99*np.log(counter/np.max(counter))

    # uncomment those in case of doubt
#    print(reslist)
#    print(counter)
#    print(nroot)

    # get the data into the proper shape for plotting with matplotlib.pyplot.matshow
    nroot_contour = np.transpose(np.reshape(nroot, (num_x, num_y)))

    # timing to see difference in time used between calculation and plotting
    print('Finished computation at '+str(datetime.datetime.now()))

    # create an imshow plot
    plt.figure()

    # label the axes
    plt.xlabel("$Re(z)$", fontsize=16)
    plt.ylabel("$Im(z)$", fontsize=16)

    # plots the matrix of data in the current figure. Interpolation isn't wanted here.
    # Change color map (cmap) for various nice looks of your fractal
    plt.matshow(nroot_contour, fignum=0, interpolation='none',
                origin='lower', cmap='hot')

    # remove ticks and tick labels from the figure
    plt.xticks([])
    plt.yticks([])

    # save a file of you plot. 200 dpi is good to match 1000 plot points.
    # Increasing dpi produces more pixels, but not more plotting points.
    # Number of plotting points and pixels are best matched by inspection.
    plt.savefig('newton-fractal-example-plot-'+func_string +
                '.jpg', bbox_inches='tight', dpi=200)

    plt.close()
    # timing step
    print('Finished creating matshow plot at '+str(datetime.datetime.now()))


# call the solution function for func of your choice
# also, switch shading via number of iterations on or off
plot_newton_fractal('npe1', perfom_shading=True)

# final timing check
print('Finished computation and plotting at '+str(datetime.datetime.now()))
