package com.rvnovae.k8sgr4.k8sgr4app

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class K8sGr4AppApplication

fun main(args: Array<String>) {
	runApplication<K8sGr4AppApplication>(*args)
}
