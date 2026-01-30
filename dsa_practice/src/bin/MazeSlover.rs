#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Point{
    x:i32,
    y:i32,
}

pub fn walk(maze:&[&str],wall:char,curr:Point,end:Point,seen:&mut Vec<Vec<bool>>,path:&mut Vec<Point>,)->bool{
    //Base case
    //1.OFF THE MAP
    if curr.x<0||curr.x>=maze[0].len() as i32||
       curr.y<0||curr.x>=maze.len() as i32{
        return false;
       }
    //it s a wall
    if maze[curr.y as usize].as_bytes()[curr.x as usize] == wall as u8{
        return false;
    }


    if curr.x == end.x && curr.y == end.y {
        path.push(end);
        return true;
    }

    if seen[curr.y as usize][curr.x as usize ] {
        return false;
    }

    //recursion
    //pre
    path.push(curr);
    seen[curr.y as usize ][curr.x as usize] = true;

    const DIR :[[i32;2];4] = [[-1,0],[1,0],[0,-1],[0,1]];

    //recurse 
    for i in 0..DIR.len(){
        let [dx,dy] = DIR[i];
        let next_point = Point{
            x:curr.x+dx,
            y:curr.y+dy,
        };
        if walk(maze,wall,next_point,end,seen,path){
            return true;
        }      
    }
    path.pop();
    return false;
}

fn main(){

}