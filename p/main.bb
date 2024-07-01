ON main(){
    may nm = "name";
    echonl("Your Name ?");
    echol("> ");
    out.flush();
    takein(nm);
    greet();
    may i = 0;
    see [i>10] => echonl("hi");
    => echonl(nm);
    => add(i,1 : i);
    echonl("loop out");
}
ON greet(){
    may times = 1;
    may timespecific = 1.0;
    echonl("Hi there ",nm," Nice to meet you!");
    echonl("");
    echol("I have greeted you ",times," times or to be specific ",timespecific," times");
}