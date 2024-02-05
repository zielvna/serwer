import Side from '../Side/Side';

export default function Sides() {
  return (
    <>
      <Side
        title="Hello world"
        description="This is the simplest server application. The routing system will attempt to match the requested URL, execute the corresponding code, and send a response to the user."
        isLeftToRight={true}
      >
        {`let mut serwer = Serwer::new();

serwer.get("/", route! {() move |_, mut res| {
    res.set(StatusCode::OK, "Hello world".to_string());
    res
}});

serwer.listen(7878);`}
      </Side>
      <Side
        title="Dynamic params"
        description="Improve your routes with dynamic parameters for a more user-friendly experience. This allows for personalized and intuitive navigation experiences for your users."
        isLeftToRight={false}
      >
        {`serwer.get("/user/<user>", route! {() move |req, mut res| {
    let user = req.param("user").unwrap_or("".to_string());
    res.set(StatusCode::OK, format!("Hello {user}"));
    res
}});`}
      </Side>
      <Side
        title="Shared data"
        description="Sharing data through your app has never been easier. Declare it once and use it across all your routes."
        isLeftToRight={true}
      >
        {`let counter = Data::new(0);

serwer.post("/click", route! {(counter) move |_, mut res| {
    let mut counter = counter.write();
    *counter += 1;
    res.set(StatusCode::OK, "Counter increased".to_string());
    res
}});

serwer.get("/counter", route! {(counter) move |_, mut res| {
    let counter = counter.read();
    res.set(StatusCode::OK, counter.to_string());
    res
}});`}
      </Side>
    </>
  );
}
