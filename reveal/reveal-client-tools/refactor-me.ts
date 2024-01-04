const getRandomResult = (): boolean => {
  return Math.random() >= 0.5;
};

const cleanRoom = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    const isRoomCleaned = getRandomResult();
    if (isRoomCleaned) {
      resolve();
      return;
    }
    reject(new Error("Didn't clean room"));
  });
};

const playFootball = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    const isGoalMade = getRandomResult();
    if (isGoalMade) {
      resolve();
      return;
    }
    reject(new Error("Didn't make goal"));
  });
};

const doLaundry = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    const isLaundryDone = getRandomResult();
    if (isLaundryDone) {
      resolve();
      return;
    }
    reject(new Error("Didn't do laundry"));
  });
};

const goToConcert = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    resolve();
  });
};

const cleanDog = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    resolve();
  });
};

const isRoomCleaned = await cleanRoom();

// If Mike is able to clean the room, then he goes for football. In football match, if he makes a goal, then as reward he gets a chance to visit the music concert. If he does not make a goal, then he cleans his dog. If he is not able to clean the room, then he does the laundry. If he is not able to finish the laundry either, then he has to clean his dog. But if he is able to clean the laundry then he goes for concert.
