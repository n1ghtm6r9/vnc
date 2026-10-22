use super::{AssemblerEvent, PendingPicture, VideoAssembler, VideoPicture};

impl VideoAssembler {
    pub(crate) fn emit(&mut self, events: &mut Vec<AssemblerEvent>) {
        let Some(mut next) = self.next else {
            return;
        };
        while self
            .pictures
            .get(&next)
            .is_some_and(|picture| picture.data.is_some())
        {
            let Some(PendingPicture {
                stripe,
                frame,
                key,
                data: Some(data),
                ..
            }) = self.pictures.remove(&next)
            else {
                break;
            };
            events.push(AssemblerEvent::Picture(VideoPicture {
                don: next,
                stripe,
                frame,
                key,
                data,
            }));
            self.stats.pictures += 1;
            next += 1;
        }
        self.next = Some(next);
    }
}
