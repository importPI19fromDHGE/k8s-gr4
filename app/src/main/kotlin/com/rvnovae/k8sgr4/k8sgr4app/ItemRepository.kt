package com.rvnovae.k8sgr4.k8sgr4app

import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository
interface ItemRepository : CrudRepository<Item, Long> {

}