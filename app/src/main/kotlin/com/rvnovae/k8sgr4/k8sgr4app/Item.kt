package com.rvnovae.k8sgr4.k8sgr4app

import javax.persistence.*

@Entity (name = "main")
class Item {
    @Id @GeneratedValue(strategy = GenerationType.IDENTITY)
    val id: Long = 0

    @Column
    val content: String = ""
}
