
extern crate queues;
// a general purpose 
// wrapper that allows you 
// to peek an iterator 
pub struct PeekWrapper<T>
where T: Iterator,
    T::Item: Clone
{
    pub iterator:T,
    peek_queue: Vec<Option<<T as Iterator>::Item>>,
    peek_size: u32,
}

impl<T> PeekWrapper<T>
where T: Iterator,
      T::Item: Clone,
{
    // generate a peek wrapper
    // given an itterator
    pub fn new(it: T, size:u32) -> PeekWrapper<T>
    where T: Iterator 
    {
        let mut peek_vec:Vec<Option<<T as Iterator>::Item>> = vec![];
        let mut iterator = it;

        // fill up the peek vec for the first time
        for i in 0..size
        {
            peek_vec.push(iterator.next());
        }

        PeekWrapper{iterator:iterator, peek_queue: peek_vec, peek_size: size}
    }

    // next
    // gets the next token 
    pub fn next(& mut self) -> Option<<T as Iterator>::Item>{
        
        let returned = self.peek_queue[0].clone();
        // shift left 
        self.shift_left();
        // get the next item from the iterator
        self.peek_queue[self.peek_size as usize -1] = self.iterator.next();

        returned
    }

    // shift_left
    // moves everything in the
    // peek vector left 
    // then calls the push func
    fn shift_left(& mut self)
    {
        for i in 0..(self.peek_size-1)
        {
            self.peek_queue[i as usize]=self.peek_queue[i as usize+1].clone();
        }
    }

    // peek
    // lets you peek into the next stuffs
    pub fn peek(&self, i: usize)-> Option<<T as Iterator>::Item>
    {
        if i as u32 >= self.peek_size
        {
            return None;
        }

        self.peek_queue[i].clone()

    }
}



// Makes it so we can loop over this 
// too
impl<T> Iterator for PeekWrapper<T> 
where T: Iterator,
      T::Item: Clone,
{
    type Item = <T as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item>
    {   
        self.next()
    }
}