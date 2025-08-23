# Introduction
The purpose of this repo is to implement an algorithm to compute the complex roots of a polynomial in a single variable using homotopy continuation on an ice40 h1xk FPGA (nandland go board). This is not intended to be a state of the art fast algorithm -- this is achieved easier by numerically computing the eigenvalues of the companion matrix.

At the moment, I have thought through the algorithm on the FPGA using a multivariable horners method. I’m still a little wary of how error propagation might occur or how to give any guarantees. I am first writing up a rust implementation and a driver of the FPGA. I have currently stopped because I'm working on other projects and couldn't work on it during the summer with other people as I hoped. 
