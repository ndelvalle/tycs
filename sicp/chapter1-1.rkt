#lang racket

(provide exercise-1-3)

(define (square a) (* a a))

(define (sum-of-squares a b) (+ (square a) (square b)))

(define (exercise-1-3 a b c)
    (cond ((> a b c) (sum-of-squares a b))
          ((> b a c) (sum-of-squares a b))
          ((> b c a) (sum-of-squares b c))
          ((> c b a) (sum-of-squares b c))
          (else (sum-of-squares a c))))

(exercise-1-3 1 2 3) ;; 13
(exercise-1-3 3 2 1) ;; 13
(exercise-1-3 3 1 2) ;; 13

;; Exercise 1.5

;; (define (p) (p))
;;
;; (define (test x y) 
;;   (if (= x 0) 
;;       0 
;;       y))
;;
;; ;; With normal evaluation
;;
;; (test 0 (p))
;;
;; (if (= 0 0)
;;     0
;;     (p))
;;
;; 0
;;
;; ;; With applicative evaluation
;;
;; ;; Attempts to evaluate p, thus loops forever
;; (test 0 (p))

;; Exercise 1.6
;;
;; Loops forever because it tries to evaluate the terms forever

(define (sqrt x)
  (define (good-enough? guess x)
    (< (abs (- (square guess) x)) 0.000001))
  (define (improve guess x)
    (average guess (/ x guess)))
  (define (sqrt-iter guess x)
    (if (good-enough? guess x)
        guess
        (sqrt-iter (improve guess x) x)))
  (define (average a b) 
    (/ (+ a b) 2))
  (sqrt-iter 1.0 x))

(sqrt (* 65536 65536)) ;; 65536.0
