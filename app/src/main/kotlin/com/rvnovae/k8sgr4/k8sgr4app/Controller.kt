package com.rvnovae.k8sgr4.k8sgr4app

import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping
class Controller(val repository: ItemRepository) {

    @GetMapping
    fun getAll(): MutableIterable<Item> = repository.findAll()

    @PostMapping
    fun addItem(@RequestBody item: Item) = repository.save(item)

    @GetMapping("/{id}")
    fun getById(@PathVariable id: Long) = repository.findById(id)

    @DeleteMapping("/{id}")
    fun deleteById(@PathVariable id: Long) = repository.deleteById(id)
}